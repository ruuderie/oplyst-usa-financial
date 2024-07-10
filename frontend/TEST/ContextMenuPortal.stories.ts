import type { Meta, StoryObj } from '@storybook/vue3';

import ContextMenuPortal from '../components/ui/context-menu/ContextMenuPortal.vue';

const meta = {
  title: 'ContextMenuPortal',
  component: ContextMenuPortal,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof ContextMenuPortal>;

export default meta;
type Story = StoryObj<typeof ContextMenuPortal>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};