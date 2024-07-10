import type { Meta, StoryObj } from '@storybook/vue3';

import MenubarGroup from '../components/ui/menubar/MenubarGroup.vue';

const meta = {
  title: 'MenubarGroup',
  component: MenubarGroup,
  tags: ['autodocs'],
  args: {}, // default value
} satisfies Meta<typeof MenubarGroup>;

export default meta;
type Story = StoryObj<typeof MenubarGroup>;

export const Primary: Story = {
    args: {
        class: 'page-container__navbar navbar container is-fluid h-28',
    }
};