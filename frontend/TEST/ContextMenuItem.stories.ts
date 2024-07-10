import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuItem from '../components/ui/context-menu/ContextMenuItem.vue';

const meta = {
  title: 'ContextMenuItem',
  component: ContextMenuItem,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuItem>;

export default meta;
type Story = StoryObj<typeof ContextMenuItem>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};