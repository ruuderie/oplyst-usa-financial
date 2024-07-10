import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuGroup from '../components/ui/context-menu/ContextMenuGroup.vue';

const meta = {
  title: 'ContextMenuGroup',
  component: ContextMenuGroup,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuGroup>;

export default meta;
type Story = StoryObj<typeof ContextMenuGroup>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};